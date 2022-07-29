import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BadGridDialogComponent } from './bad-grid-dialog.component';

describe('BadGridDialogComponent', () => {
  let component: BadGridDialogComponent;
  let fixture: ComponentFixture<BadGridDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BadGridDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BadGridDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
