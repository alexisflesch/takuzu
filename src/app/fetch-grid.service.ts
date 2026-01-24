import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, Observable, of, tap, map } from 'rxjs';

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
    '4'?: number;
    '6'?: number;
    '8'?: number;
    '10'?: number;
    '12'?: number;
    '14'?: number;
    [key: string]: number | undefined;
  }
}

interface DifficultyCounts {
  d1: number;
  d2: number;
  d3: number;
  d4: number;
}

interface Summary {
  sizes: { [size: string]: DifficultyCounts };
}

@Injectable({
  providedIn: 'root'
})

export class FetchGridService {

  constructor(
    private http: HttpClient
  ) { }

  fetchSummary(): Observable<Summary> {
    return this.http.get<Summary>(`assets/grids/grids_summary.json`).pipe(
      catchError(error => {
        console.warn('grids_summary.json not found or invalid, returning empty summary', error);
        return of({ sizes: {} });
      })
    );
  }

  // Backwards-compatible stats: sums counts per size
  fetchStats(): Observable<stats> {
    return this.fetchSummary().pipe(
      map(summary => {
        const totals: { [k: string]: number } = {};
        for (const [size, counts] of Object.entries(summary.sizes || {})) {
          const sum = (counts.d1 || 0) + (counts.d2 || 0) + (counts.d3 || 0) + (counts.d4 || 0);
          totals[size.replace('x', '')] = sum;
        }
        return { id: 1, totals } as stats;
      }),
      catchError(_ => of({ id: 1, totals: {} } as stats))
    );
  }

  fetchGrid(size: number, difficulty: number, index: number): Observable<takuzuGrid> {
    const filename = `takuzu_${size}x${size}_d${difficulty}.json`;
    return this.http.get<any[]>(`assets/grids/${filename}`).pipe(
      map(arr => {
        if (!Array.isArray(arr) || arr.length === 0) {
          throw new Error('Empty grid file');
        }
        const item = arr[index];
        if (!item) {
          throw new Error('Index out of range');
        }
        // Support both formats: { grid, solution } and { puzzle, solution }
        const rawGrid = item.grid ?? item.puzzle ?? [];
        const rawSol = item.solution ?? [];

        const normGrid = Array.isArray(rawGrid) ? rawGrid.map((row: any[]) => row.map((v: any) => (v === null ? -1 : v))) : [];
        const normSol = Array.isArray(rawSol) ? rawSol.map((row: any[]) => row.map((v: any) => (v === null ? -1 : v))) : [];

        return {
          grid: normGrid,
          solution: normSol,
          id: item.id ?? `grid_${index + 1}`
        } as takuzuGrid;
      }),
      catchError(error => {
        console.error('Error loading grid:', error);
        return of({ grid: [], solution: [], id: '' });
      })
    );
  }

}
