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

  public shareUrl: string = '';

  constructor(
    public dialogRef: MatDialogRef<CheckDialogComponent>,
    @Inject(MAT_DIALOG_DATA) public data: { fullStars: number[], halfStars: number[], emptyStars: number[], size?: number, difficulty?: number, currentIndex?: number, tot?: number },
  ) {
    // Build a share URL with a new random index different from currentIndex
    if (data.size !== undefined && data.difficulty !== undefined && data.tot !== undefined) {
      let newIndex = Math.floor(Math.random() * data.tot);
      if (data.currentIndex !== undefined && data.tot > 1) {
        while (newIndex === data.currentIndex) {
          newIndex = Math.floor(Math.random() * data.tot);
        }
      }
      this.shareUrl = `${window.location.origin}/play?size=${data.size}&difficulty=${data.difficulty}&index=${newIndex}`;
    }
  }

  onClickNo(): void {
    this.dialogRef.close("no");
  }
  onClickYes(): void {
    this.dialogRef.close("yes");
  }

}