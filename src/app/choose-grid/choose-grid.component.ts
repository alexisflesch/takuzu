import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { FetchGridService } from '../fetch-grid.service';

type gridChoice = ('4' | '6' | '8' | '10' | '12' | '14')

@Component({
  selector: 'app-choose-grid',
  templateUrl: './choose-grid.component.html',
  styleUrls: ['./choose-grid.component.scss']
})
export class ChooseGridComponent implements OnInit {

  stats = {
    'id': 0,
    'totals': {
      '4': 1, '6': 1, '8': 1, '10': 1, '12': 1, '14': 1
    }
  }

  gridSizes: string[] = ['4', '6', '8', '10']//, '12', '14']
  gridSizeChoice: gridChoice = '4'

  constructor(
    private fetchGridService: FetchGridService,
    private router: Router
  ) { }

  ngOnInit(): void {
    this.fetchGridService.fetchStats().subscribe(
      response => {
        this.stats = response;
      }
    )
  }

  playGrid(size: string) {
    const num = Math.floor(Math.random() * this.stats['totals'][this.gridSizeChoice]);
    const id = size + '-' + num
    this.router.navigate(['/play'], { queryParams: { 'tot': this.stats['totals'][this.gridSizeChoice], id } })
  }

}
