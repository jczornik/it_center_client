import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MessageDashboardModule } from "./message-dashboard/message-dashboard.module";
import { MatSidenavModule } from "@angular/material/sidenav";
import { MatToolbarModule } from "@angular/material/toolbar";
import { MatButtonModule } from "@angular/material/button";

@NgModule({
    declarations: [AppComponent],
    imports: [BrowserModule, BrowserAnimationsModule, MessageDashboardModule, MatSidenavModule, MatToolbarModule, MatButtonModule],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule { }
