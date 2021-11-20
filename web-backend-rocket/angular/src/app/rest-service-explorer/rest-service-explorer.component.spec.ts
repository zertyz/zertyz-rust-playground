import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RestServiceExplorerComponent } from './rest-service-explorer.component';

describe('RestServiceComponent', () => {
  let component: RestServiceExplorerComponent;
  let fixture: ComponentFixture<RestServiceExplorerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ RestServiceExplorerComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(RestServiceExplorerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
