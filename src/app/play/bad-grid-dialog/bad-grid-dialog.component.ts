import { Component } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';

@Component({
  selector: 'app-bad-grid-dialog',
  templateUrl: './bad-grid-dialog.component.html',
  styleUrls: ['./bad-grid-dialog.component.scss']
})
export class BadGridDialogComponent {

  constructor(
    public dialogBadGrid: MatDialogRef<BadGridDialogComponent>,
  ) { }

  close(): void {
    this.dialogBadGrid.close()
  }

}
