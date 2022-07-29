import { TestBed } from '@angular/core/testing';

import { FetchGridService } from './fetch-grid.service';

describe('FetchGridService', () => {
  let service: FetchGridService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(FetchGridService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
