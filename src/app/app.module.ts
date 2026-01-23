import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HomeComponent } from './home/home.component';
import { ChooseGridComponent } from './choose-grid/choose-grid.component';
import { PlayComponent } from './play/play.component';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatListModule } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';
import { MatGridListModule } from '@angular/material/grid-list';
import { TakuzuItemStylePipe } from './takuzu-item-style.pipe';
import { HttpClientModule } from '@angular/common/http';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatRadioModule } from '@angular/material/radio';
import { FormsModule } from '@angular/forms';
import { MatDialogModule } from '@angular/material/dialog';
import { HelpDialogComponent } from './play/help-dialog/help-dialog.component';
import { CheckDialogComponent } from './play/check-dialog/check-dialog.component';
// import { ServiceWorkerModule } from '@angular/service-worker';
import { environment } from '../environments/environment';
import { NewGameDialogComponent } from './play/new-game-dialog/new-game-dialog.component';
import { MatSelectModule } from '@angular/material/select';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatBadgeModule } from '@angular/material/badge';
import { BulbPipe } from './bulb.pipe';
import { BadGridDialogComponent } from './play/bad-grid-dialog/bad-grid-dialog.component';



@NgModule({
  declarations: [
    AppComponent,
    HomeComponent,
    ChooseGridComponent,
    PlayComponent,
    TakuzuItemStylePipe,
    HelpDialogComponent,
    CheckDialogComponent,
    NewGameDialogComponent,
    BulbPipe,
    BadGridDialogComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MatToolbarModule,
    MatIconModule,
    MatSidenavModule,
    MatListModule,
    MatButtonModule,
    MatGridListModule,
    MatProgressSpinnerModule,
    MatRadioModule,
    FormsModule,
    MatDialogModule,
    MatBadgeModule,
    MatSelectModule,
    MatFormFieldModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
