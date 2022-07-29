import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Router, ActivatedRoute } from '@angular/router';
import { FetchGridService } from '../fetch-grid.service';
import { CheckDialogComponent } from './check-dialog/check-dialog.component';
import { HelpDialogComponent } from './help-dialog/help-dialog.component';
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
  //Score stuff
  fullStars: number = 0
  halfStars: number = 0
  emptyStars: number = 0

  //Animation
  takuzuGridAnimation: number[] = []


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
      this.tot = params['tot']
      var gridId = params['id']
      if (!gridId) {
        gridId = '-1'
      }
      this.fetchGridService.fetchGrid(gridId).subscribe(
        response => {
          this.takuzuGrid = response['grid'].flat()
          this.takuzuSolution = response['solution'].flat()
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
    const num = Math.floor(Math.random() * this.tot);
    const id = this.takuzuSize + '-' + num
    this.takuzuGrid = []
    this.router.navigate(['/play'], { queryParams: { 'tot': this.tot, id } })
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
    if (this.nbClues == 0 && this.completed && !this.solved) {
      const dialogBadGrid = this.dialogBadGrid.open(BadGridDialogComponent)
      return
    }
    else if (this.nbClues == 0) {
      return
    }
    else {
      const dialogHelpRef = this.dialogHelp.open(HelpDialogComponent);
      dialogHelpRef.afterClosed().subscribe(result => {
        if (result == 'no') {
          return
        }
        const errors = this.getErrors()
        const indexes = this.getEmptySpots()

        var foo = 0
        //S'il y a des erreurs on en corrige une avec proba 0.5
        if (errors.length && indexes.length) {
          foo = Math.random()
        }
        //Si la grille est pleine on cherche les erreurs
        else if (!indexes.length) {
          foo = 1
        }
        if (foo < .5) {
          var randomIndex = Math.floor(Math.random() * indexes.length)
          this.takuzuGrid[indexes[randomIndex]] = this.takuzuSolution[indexes[randomIndex]] + 4
          this.nbClues -= 1;
          this.checkIfCompleted()
        }
        else {
          this.nbClues -= 1
          if (errors) {
            var randomIndex = Math.floor(Math.random() * errors.length)
            this.takuzuGrid[errors[randomIndex]] = this.takuzuSolution[errors[randomIndex]] + 4
            this.checkIfCompleted()
          }
        }
      });
    }
  }

  clickUndo(): void {
    // Go back in history when undo button is clicked
    let i = this.history.pop()
    if (i) {
      if (this.takuzuGrid[i] == 2) {
        this.takuzuGrid[i] = -1
        this.completed = false
        this.solved = false
      }
      else if (this.takuzuGrid[i] == 3) {
        this.takuzuGrid[i] = 2
        this.checkIfCompleted()
      }
      else if (this.takuzuGrid[i] == -1) {
        this.takuzuGrid[i] = 3
        this.checkIfCompleted()
      }
      else {
        //When help button has been used and an item has been overwritten
        this.clickUndo()
      }
    }
  }


  clickSquare(index: number): void {
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


