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
    if (!counts) {
      this.difficulties = [];
      return;
    }
    const diffs: number[] = [];
    for (let d = 1; d <= 5; d++) {
      if ((counts as any)[`d${d}`] && (counts as any)[`d${d}`] > 0) {
        diffs.push(d);
      }
    }
    this.difficulties = diffs;
    if (diffs.length) {
      this.difficultyChoice = diffs[0];
    }
  }

  playGrid() {
    const sizeKey = `${this.gridSizeChoice}x${this.gridSizeChoice}`;
    const counts = (this.summary.sizes || {})[sizeKey];
    if (!counts) {
      return;
    }
    const tot = (counts as any)[`d${this.difficultyChoice}`] || 0;
    if (!tot) {
      return;
    }
    const randomIndex = Math.floor(Math.random() * tot);
    this.router.navigate(['/play'], { queryParams: { size: this.gridSizeChoice, difficulty: this.difficultyChoice, index: randomIndex } })
  }

}
