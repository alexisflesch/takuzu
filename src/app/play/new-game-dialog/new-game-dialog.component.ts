import { Component } from '@angular/core';

//Import obligatoire pour styliser les boutons même si vscode
//n'a pas l'air d'y croire.
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';


@Component({
  selector: 'app-new-game-dialog',
  templateUrl: './new-game-dialog.component.html',
  styleUrls: ['./new-game-dialog.component.scss']
})
export class NewGameDialogComponent {

  constructor(
    public dialogRef: MatDialogRef<NewGameDialogComponent>,
  ) { }


  onClickNo(): void {
    this.dialogRef.close("no");
  }
  onClickYes(): void {
    this.dialogRef.close("yes");
  }

}
