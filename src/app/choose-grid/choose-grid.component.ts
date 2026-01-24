import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { FetchGridService } from '../fetch-grid.service';

@Component({
  selector: 'app-choose-grid',
  templateUrl: './choose-grid.component.html',
  styleUrls: ['./choose-grid.component.scss']
})
export class ChooseGridComponent implements OnInit {

  gridSizes: string[] = []
  gridSizeChoice: string = ''
  difficulties: number[] = []
  difficultyChoice: number = 1
  hasPuzzles: boolean = false

  private summary: any = { sizes: {} }

  constructor(
    private router: Router,
    private fetchGridService: FetchGridService
  ) { }

  ngOnInit(): void {
    this.fetchGridService.fetchSummary().subscribe(summary => {
      this.summary = summary || { sizes: {} };
      this.gridSizes = Object.keys(this.summary.sizes || {}).sort((a, b) => parseInt(a) - parseInt(b));
      if (this.gridSizes.length) {
        this.gridSizeChoice = this.gridSizes[0].split('x')[0];
        this.updateDifficulties();
      }
    });
  }

  updateDifficulties(): void {
    const sizeKey = `${this.gridSizeChoice}x${this.gridSizeChoice}`;
    const counts = (this.summary.sizes || {})[sizeKey];
    // Always offer difficulties 1..4 (no more)
    this.difficulties = [1, 2, 3, 4];
    // Keep current selection if valid, otherwise default to 1
    if (!this.difficulties.includes(this.difficultyChoice)) {
      this.difficultyChoice = 1;
    }

    // Track whether there are any puzzles for this size (consider d1..d4)
    this.hasPuzzles = !!counts && ((counts.d1 || 0) + (counts.d2 || 0) + (counts.d3 || 0) + (counts.d4 || 0) > 0);
  }

  playGrid() {
    const sizeKey = `${this.gridSizeChoice}x${this.gridSizeChoice}`;
    const counts = (this.summary.sizes || {})[sizeKey];
    if (!counts) {
      return;
    }
    // Silently map size 4 + difficulty 4 to difficulty 3
    let effectiveDifficulty = this.difficultyChoice;
    if (parseInt(this.gridSizeChoice, 10) === 4 && this.difficultyChoice === 4) {
      effectiveDifficulty = 3;
    }
    const tot = (counts as any)[`d${effectiveDifficulty}`] || 0;
    if (!tot) {
      return;
    }
    const randomIndex = Math.floor(Math.random() * tot);
    // Keep the user's chosen difficulty in the query params (we'll map it again when loading)
    this.router.navigate(['/play'], { queryParams: { size: this.gridSizeChoice, difficulty: this.difficultyChoice, index: randomIndex } })
  }

}
