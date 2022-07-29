import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ChooseGridComponent } from './choose-grid.component';

describe('ChooseGridComponent', () => {
  let component: ChooseGridComponent;
  let fixture: ComponentFixture<ChooseGridComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ChooseGridComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ChooseGridComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
