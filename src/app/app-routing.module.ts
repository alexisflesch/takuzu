import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ChooseGridComponent } from './choose-grid/choose-grid.component';
import { HomeComponent } from './home/home.component';
import { PlayComponent } from './play/play.component';

const routes: Routes = [
  { path: 'home', component: HomeComponent },
  { path: 'choose-grid', component: ChooseGridComponent },
  { path: 'play', component: PlayComponent },
  { path: '', redirectTo: '/home', pathMatch: 'full' },
  { path: '**', component: HomeComponent }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
