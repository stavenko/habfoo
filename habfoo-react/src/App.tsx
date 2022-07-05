import React from "react";
import { observer } from "mobx-react-lite";
import { AppState, CurrentView } from "./store/app-state";
import {NewFoodItemComponent, FoodFormState} from "./components/new-food-item";

interface Props {
  state: AppState;
}

export default observer(({ state }: Props) => {
  console.log("app", state.currentView);
  if (state.currentView === CurrentView.CreateFood) {
    return <NewFoodItemComponent state={ new FoodFormState() } />;
  }
  
});
