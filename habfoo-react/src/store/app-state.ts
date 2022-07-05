// import {DefaultApi, ServerConfiguration} from 'habfoo-api';
import {makeObservable, observable, action} from 'mobx';

export enum CurrentView {
  Main,
  CreateFood
}

export class AppState {
  currentView: CurrentView;
  barCode?: string;
  // api: DefaultApi;
  constructor() {
    this.currentView = CurrentView.CreateFood;
    this.barCode = undefined;
    // const baseServer = new ServerConfiguration("http://localhost", {});
    // this.api = new DefaultApi({baseServer, })
    makeObservable(this, {
      currentView: observable,
      barCode: observable,
      updateBarcode: action.bound
    });
  }

  updateBarcode(s: string) {
    this.barCode = s
  }
}
