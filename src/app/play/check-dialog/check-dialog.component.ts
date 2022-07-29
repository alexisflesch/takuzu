import { Component, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

//Import obligatoire pour styliser les boutons même si vscode
//n'a pas l'air d'y croire.
import { MatButtonModule } from '@angular/material/button';


@Component({
  selector: 'app-check-dialog',
  templateUrl: './check-dialog.component.html',
  styleUrls: ['./check-dialog.component.scss']
})
export class CheckDialogComponent {

  constructor(
    public dialogRef: MatDialogRef<CheckDialogComponent>,
    @Inject(MAT_DIALOG_DATA) public data: { fullStars: number[], halfStars: number[], emptyStars: number[] },
  ) { }

  onClickNo(): void {
    this.dialogRef.close("no");
  }
  onClickYes(): void {
    this.dialogRef.close("yes");
  }

}