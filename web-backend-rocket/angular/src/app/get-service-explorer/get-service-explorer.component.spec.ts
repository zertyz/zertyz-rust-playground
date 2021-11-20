import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GetServiceExplorerComponent } from './get-service-explorer.component';

describe('GetServiceComponent', () => {
  let component: GetServiceExplorerComponent;
  let fixture: ComponentFixture<GetServiceExplorerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ GetServiceExplorerComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(GetServiceExplorerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
