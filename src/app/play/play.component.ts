import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Router, ActivatedRoute } from '@angular/router';
import { FetchGridService } from '../fetch-grid.service';
import { CheckDialogComponent } from './check-dialog/check-dialog.component';

import { NewGameDialogComponent } from './new-game-dialog/new-game-dialog.component';


import {
  trigger,
  state,
  style,
  animate,
  transition,
  keyframes,
} from '@angular/animations';
import { BadGridDialogComponent } from './bad-grid-dialog/bad-grid-dialog.component';


@Component({
  selector: 'app-play',
  templateUrl: './play.component.html',
  styleUrls: ['./play.component.scss'],
  animations: [
    trigger('changeColor', [
      state('0', style({
        background: 'lightblue',
        border: 'solid 2px navy',
      })),
      state('1', style({
        background: 'lightseagreen',
        border: 'solid 2px navy',
      })),
      state('4', style({
        background: 'lightblue',
        border: 'solid 2px firebrick',
      })),
      state('5', style({
        background: 'lightseagreen',
        border: 'solid 2px firebrick',
      })),
      transition('* => 0', [
        animate('.5s', keyframes([
          style({ background: 'navy' })
        ])
        ), animate('.5s')
      ]),
      transition('* => 1', [
        animate('.5s', keyframes([
          style({ background: 'navy' })
        ])
        ), animate('.5s')
      ]),
      transition('* => 4', [
        animate('.5s', keyframes([
          style({ background: 'firebrick' })
        ])
        ), animate('.5s')
      ]),
      transition('* => 5', [
        animate('.5s', keyframes([
          style({ background: 'firebrick' })
        ])
        ), animate('.5s')
      ]),
    ]),
  ]
})
export class PlayComponent implements OnInit {


  takuzuGrid: number[] = []
  takuzuSolution: number[] = []
  indexes: number[] = []
  takuzuId: string = '0'
  takuzuSize: number = 0
  history: number[] = []
  completed: boolean = false
  solved: boolean = false
  tot: number = 1 //Total number of grids of this size (to create a new game)
  nbClues: number = 0; //Number of available clues
  isFlashing: boolean = false; // When true, clicking a square will reveal it (help mode)
  //Score stuff
  fullStars: number = 0
  halfStars: number = 0
  emptyStars: number = 0

  //Animation
  takuzuGridAnimation: number[] = []

  // Track current grid parameters so we can start a new game with same size/difficulty
  currentSize: number = 0
  currentDifficulty: number = 1
  currentIndex: number = -1


  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private fetchGridService: FetchGridService,
    public dialogHelp: MatDialog,
    public dialogCheck: MatDialog,
    public dialogNewGame: MatDialog,
    public dialogBadGrid: MatDialog,
  ) { }

  ngOnInit(): void {
    //Get stats for a new game
    this.route.queryParams.subscribe(params => {
      const size = parseInt(params['size']);
      const difficulty = parseInt(params['difficulty']);
      const index = parseInt(params['index']);

      // Validate params
      if (isNaN(size) || isNaN(difficulty) || isNaN(index)) {
        // Redirect to chooser if params are invalid
        this.router.navigate(['/choose-grid']);
        return;
      }

      this.currentSize = size
      this.currentDifficulty = difficulty
      this.currentIndex = index

      // Determine effective difficulty (map size 4 + difficulty 4 -> difficulty 3)
      let effectiveDifficulty = difficulty;
      if (size === 4 && difficulty === 4) {
        effectiveDifficulty = 3;
      }

      // Get total available puzzles for the chosen size/difficulty
      this.fetchGridService.fetchSummary().subscribe(summary => {
        const sizeKey = `${size}x${size}`;
        const counts = (summary.sizes || {})[sizeKey] || { d1: 0, d2: 0, d3: 0, d4: 0, d5: 0 };
        this.tot = (counts as any)[`d${effectiveDifficulty}`] || 0;

        // If no puzzles available, redirect back to chooser
        if (!this.tot || this.tot <= 0) {
          this.router.navigate(['/choose-grid']);
          return;
        }

        // If requested index is invalid, pick a random one
        const idx = (isNaN(index) || index < 0 || index >= this.tot) ? Math.floor(Math.random() * this.tot) : index;
        this.currentIndex = idx;

        this.fetchGridService.fetchGrid(size, effectiveDifficulty, idx).subscribe(
          response => {
            // Guard against missing or malformed responses
            if (!response || !response['grid'] || !response['solution']) {
              console.error('Grid not found or malformed:', size, difficulty, idx, response);
              this.router.navigate(['/choose-grid']);
              return;
            }

            const gridArray = Array.isArray(response['grid']) ? response['grid'].flat() : [];
            const solArray = Array.isArray(response['solution']) ? response['solution'].flat() : [];

            if (gridArray.length === 0 || solArray.length === 0) {
              console.error('Grid or solution empty:', size, difficulty, idx);
              this.router.navigate(['/choose-grid']);
              return;
            }

            this.takuzuGrid = gridArray
            this.takuzuSolution = solArray
            this.takuzuId = response['id']
            this.takuzuSize = Math.sqrt(this.takuzuGrid.length)
            this.nbClues = this.takuzuSize - 2
            this.indexes = Array(this.takuzuGrid.length).fill(1).map((x, i) => i)
            this.solved = false
            this.completed = false
            //Animation grid
            this.takuzuGridAnimation = Array(this.takuzuGrid.length)
            for (let i = 0; i < this.takuzuGrid.length; i++) {
              if (this.takuzuGrid[i] != -1) {
                this.takuzuGridAnimation[i] = 1
              }
            }
          }
        )
      })
    });

  }

  getEmptySpots(): number[] {
    // indexes of empty spots in the grid
    var indexes = [], i;
    for (i = 0; i < this.takuzuGrid.length; i++) {
      if (this.takuzuGrid[i] === -1) {
        indexes.push(i);
      }
    }
    return indexes
  }

  getErrors(): number[] {
    //Get indexes of mistakes
    var errors: number[] = []
    for (let i = 0; i < this.takuzuGrid.length; i++) {
      if ((this.takuzuGrid[i] == 2 && this.takuzuSolution[i] == 1)
        || (this.takuzuGrid[i] == 3 && this.takuzuSolution[i] == 0)
      ) {
        errors.push(i)
      }
    }
    return errors
  }

  removeErrors(): void {
    for (let i = 0; i < this.takuzuGrid.length; i++) {
      if (this.takuzuGrid[i] == 2 && this.takuzuSolution[i] == 1) {
        this.takuzuGrid[i] = 5
      }
      else if (this.takuzuGrid[i] == 3 && this.takuzuSolution[i] == 0) {
        this.takuzuGrid[i] = 4
      }
    }
  }

  checkIfCompleted(): void {
    // Si la grille est terminée on la vérifie
    if (this.takuzuGrid.indexOf(-1) === -1) {
      this.completed = true
      var errors: number[] = this.getErrors()
      if (errors.length) {
        this.solved = false
      }
      else {
        this.solved = true
        this.youWin()
      }
    }
  }


  computeScore(): void {
    // Calcul du score
    var totClues = this.takuzuSize - 2
    var unusedClues = this.nbClues
    //Score sur 10, minimum : 2.
    var doubleScore = Math.floor(unusedClues / totClues * 4 * 2 + 2)
    this.fullStars = Math.floor(doubleScore / 2)
    this.halfStars = doubleScore - 2 * this.fullStars
    this.emptyStars = 5 - this.fullStars - this.halfStars
  }


  youWin(): void {
    this.computeScore()

    const dialogCheckRef = this.dialogCheck.open(CheckDialogComponent,
      {
        data: {
          fullStars: Array(this.fullStars).fill(0),
          halfStars: Array(this.halfStars).fill(0),
          emptyStars: Array(this.emptyStars).fill(0),
          size: this.currentSize,
          difficulty: this.currentDifficulty,
          currentIndex: this.currentIndex,
          tot: this.tot
        }
      }
    );
    dialogCheckRef.afterClosed().subscribe(result => {
      if (result == 'yes') {
        this.startNewGame()
      }
    });
  }

  startNewGame(): void {
    // Pick a new random index (0..tot-1) different from currentIndex
    let num = Math.floor(Math.random() * this.tot);
    if (this.currentIndex >= 0 && this.tot > 1) {
      while (num === this.currentIndex) {
        num = Math.floor(Math.random() * this.tot);
      }
    }

    this.takuzuGrid = []
    // Navigate keeping same size and difficulty and the new index
    this.router.navigate(['/play'], { queryParams: { size: this.currentSize, difficulty: this.currentDifficulty, index: num } })
  }

  clickNewGame(): void {
    const dialogNewGameRef = this.dialogCheck.open(NewGameDialogComponent);
    dialogNewGameRef.afterClosed().subscribe(result => {
      if (result == 'yes') {
        this.startNewGame()
      }
    });
  }

  clickHelp(): void {
    // If no clues left, button is disabled in template; do nothing as safeguard
    if (this.nbClues == 0) {
      return
    }

    // Toggle flashing mode: when flashing, clicking an empty square will reveal it
    this.isFlashing = !this.isFlashing
  }

  clickUndo(): void {
    // Go back in history when undo button is clicked
    let i = this.history.pop()

    // Pop history until we find an index we can actually revert, and
    // handle index 0 correctly by checking against undefined rather than falsiness.
    while (typeof i !== 'undefined') {
      if (this.takuzuGrid[i] == 2) {
        this.takuzuGrid[i] = -1
        this.completed = false
        this.solved = false
        return
      }
      else if (this.takuzuGrid[i] == 3) {
        this.takuzuGrid[i] = 2
        this.checkIfCompleted()
        return
      }
      else if (this.takuzuGrid[i] == -1) {
        this.takuzuGrid[i] = 3
        this.checkIfCompleted()
        return
      }
      else {
        // When help button has been used and an item has been overwritten,
        // skip this history entry and continue with the next one.
        i = this.history.pop()
      }
    }
  }


  clickSquare(index: number): void {
    // If in flashing/help mode: reveal an empty square and consume one clue
    if (this.isFlashing) {
      // Only reveal empty, modifiable squares
      if (this.takuzuGrid[index] === -1) {
        // Reveal the real value and make it read-only (same behavior as initial given squares)
        this.takuzuGrid[index] = this.takuzuSolution[index]
        this.nbClues -= 1
        this.isFlashing = false
        this.checkIfCompleted()
        return
      } else {
        // Clicking anywhere else cancels flashing
        this.isFlashing = false
        return
      }
    }

    // Si case non modifiable
    if (this.takuzuGrid[index] == 0 || this.takuzuGrid[index] == 1 ||
      this.takuzuGrid[index] == 4 || this.takuzuGrid[index] == 5
    ) {
      return
    }
    // On enregistre dans l'historique
    this.history.push(index)
    // Si case vaut 3, on la vide
    if (this.takuzuGrid[index] == 3) {
      this.takuzuGrid[index] = -1
      this.completed = false
    }
    // Sinon si vide alors 2 et si 2 alors 3.
    else {
      this.takuzuGrid[index] = (this.takuzuGrid[index] + 1) % 2 + 2
      this.checkIfCompleted()
    }
  }

}


