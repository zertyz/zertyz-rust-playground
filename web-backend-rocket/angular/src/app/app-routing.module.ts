import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from "./home/home.component";
import { RestServiceExplorerComponent } from "./rest-service-explorer/rest-service-explorer.component";
import { GetServiceExplorerComponent } from "./get-service-explorer/get-service-explorer.component";
import { PostServiceExplorerComponent } from "./post-service-explorer/post-service-explorer.component";

const routes: Routes = [
  { path: '', redirectTo: '/home', pathMatch: 'full' },
  { path: 'home', component: HomeComponent },
  { path: 'rest-service-explorer', component: RestServiceExplorerComponent },
  { path: 'get-service-explorer', component: GetServiceExplorerComponent },
  { path: 'post-service-explorer', component: PostServiceExplorerComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
