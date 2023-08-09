import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MessageDashboardComponent } from './message-dashboard.component';

describe('MessageDashboardComponent', () => {
  let component: MessageDashboardComponent;
  let fixture: ComponentFixture<MessageDashboardComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ MessageDashboardComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MessageDashboardComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
