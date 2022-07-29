import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { catchError, Observable, of, tap } from 'rxjs';
import { isDevMode } from '@angular/core';

// Convention pour les cases :
// -1 : case vide
// 0 : case initialisée à 0
// 1 : case initialisée à 1
// 2 : case remplie par l'utilisateur à 0
// 3 : case remplie par l'utilisateur à 1
// 4 : case remplie par le jeu (aide) à 0
// 5 : case remplie par le jeu (aide) à 1

interface takuzuGrid {
  grid: number[][];
  solution: number[][];
  id: string;
}

interface stats {
  id: number;
  totals: {
    '4': number;
    '6': number;
    '8': number;
    '10': number;
    '12': number;
    '14': number;
  }
}

@Injectable({
  providedIn: 'root'
})


export class FetchGridService {

  constructor(
    private http: HttpClient
  ) { }


  fetchStats(): Observable<stats> {
    if (isDevMode()) {
      return this.http.get<stats>('http://localhost:4200/api/stats')
    }
    else {
      return this.http.get<stats>('https://takuzuapi.alexisfles.ch/api/stats')
    }
  }

  fetchGrid(id: string): Observable<takuzuGrid> {
    if (isDevMode()) {
      return this.http.get<takuzuGrid>('http://localhost:4200/api/grids/' + id)
    }
    else {
      return this.http.get<takuzuGrid>('https://takuzuapi.alexisfles.ch/api/grids/' + id)
    }
  }

}
