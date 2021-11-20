import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PostServiceExplorerComponent } from './post-service-explorer.component';

describe('PostServiceComponent', () => {
  let component: PostServiceExplorerComponent;
  let fixture: ComponentFixture<PostServiceExplorerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PostServiceExplorerComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(PostServiceExplorerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
