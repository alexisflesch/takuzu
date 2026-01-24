import { ComponentFixture, TestBed } from '@angular/core/testing';
import { of } from 'rxjs';
import { ActivatedRoute } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { NO_ERRORS_SCHEMA } from '@angular/core';
import { TakuzuItemStylePipe } from '../takuzu-item-style.pipe';
import { BulbPipe } from '../bulb.pipe';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

import { PlayComponent } from './play.component';
import { FetchGridService } from '../fetch-grid.service';

describe('PlayComponent', () => {
  let component: PlayComponent;
  let fixture: ComponentFixture<PlayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PlayComponent, TakuzuItemStylePipe, BulbPipe ],
      imports: [ NoopAnimationsModule ],
      providers: [
        { provide: ActivatedRoute, useValue: { queryParams: of({ size: '6', difficulty: '1', index: '0' }) } },
        { provide: FetchGridService, useValue: {
          fetchSummary: () => of({ sizes: { '6x6': { d1: 1 } } }),
          fetchGrid: () => of({ grid: [[-1,-1,-1,-1,-1,-1]], solution: [[0,1,0,1,0,1]], id: 'test' })
        } },
        { provide: MatDialog, useValue: { open: () => ({ afterClosed: () => of(null) }) } }
      ],
      schemas: [ NO_ERRORS_SCHEMA ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PlayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('undo should revert a change at index 0 (regression test)', () => {
    // Arrange
    component.takuzuGrid = [-1, -1, -1];
    component.takuzuSolution = [0, 1, 0];

    // Act: make a change at index 0 and then undo it
    component.clickSquare(0); // history: [0], grid[0] -> 2
    expect(component.history.length).toBe(1);
    expect(component.takuzuGrid[0]).toBe(2);

    component.clickUndo(); // expected: grid[0] -> -1

    // Assert
    expect(component.takuzuGrid[0]).toBe(-1);
    expect(component.history.length).toBe(0);
  });

  it('multiple undos including index 0 should fully revert state', () => {
    // Arrange
    component.takuzuGrid = [-1, -1];

    // Act: perform toggles
    component.clickSquare(0); // history [0], grid0 -> 2
    component.clickSquare(1); // history [0,1], grid1 -> 2
    component.clickSquare(0); // history [0,1,0], grid0 -> 3

    expect(component.history.length).toBe(3);
    expect(component.takuzuGrid).toEqual([3, 2]);

    // Undo 3 times
    component.clickUndo(); // undo idx 0 -> should set grid0 to 2
    component.clickUndo(); // undo idx 1 -> should set grid1 to -1
    component.clickUndo(); // undo idx 0 -> should set grid0 to -1

    // Assert final state is fully reverted
    expect(component.takuzuGrid).toEqual([-1, -1]);
    expect(component.history.length).toBe(0);
  });
});
