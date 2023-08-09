import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MessageDashboardComponent } from './message-dashboard/message-dashboard.component';

import { MatExpansionModule } from "@angular/material/expansion"



@NgModule({
    declarations: [
        MessageDashboardComponent
    ],
    imports: [
        CommonModule,
        MatExpansionModule,
    ],
    exports: [MessageDashboardComponent],
})
export class MessageDashboardModule { }
