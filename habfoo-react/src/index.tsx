import React from "react";
import { render } from "react-dom";
import Application from "./App";
import {AppState} from './store/app-state';

render(<Application state={ new AppState() } />, document.body);
