import { ComponentFixture, TestBed } from '@angular/core/testing';
import { of } from 'rxjs';

import { ChooseGridComponent } from './choose-grid.component';
import { FetchGridService } from '../fetch-grid.service';

describe('ChooseGridComponent', () => {
  let component: ChooseGridComponent;
  let fixture: ComponentFixture<ChooseGridComponent>;

  const fetchStub = {
    fetchSummary: () => of({ sizes: {} })
  };

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ChooseGridComponent],
      providers: [{ provide: FetchGridService, useValue: fetchStub }]
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
