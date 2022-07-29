import { Component, OnInit } from '@angular/core';
import { MatDialogRef, MatDialog } from '@angular/material/dialog';

//Import obligatoire pour styliser les boutons même si vscode
//n'a pas l'air d'y croire.
import { MatButtonModule } from '@angular/material/button';


@Component({
  selector: 'app-help-dialog',
  templateUrl: './help-dialog.component.html',
  styleUrls: ['./help-dialog.component.scss']
})
export class HelpDialogComponent implements OnInit {

  helpType: string = 'fillGrid'

  constructor(
    public dialogRef: MatDialogRef<HelpDialogComponent>,
  ) { }

  ngOnInit(): void {
    this.dialogRef.keydownEvents().subscribe(event => {
      if (event.key === "Escape") {
        this.onClickNo();
      }
    });

  }
  onClickNo(): void {
    this.dialogRef.close("no");
  }
  onClickYes(): void {
    this.dialogRef.close(this.helpType);
  }

}