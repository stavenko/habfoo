import React  from "react";
import { makeObservable, observable, action } from "mobx";
import {
  FoodItemInner,
  FundamentalFoodItem,
  Nutrient,
  NutrientKindEnum,
  DefaultApi
} from "habfoo-api";
import { css, injectGlobal } from "@emotion/css";
import { observer } from "mobx-react-lite";

export class FoodFormState {
  api: DefaultApi;
  title: string;
  brand: string;
  barcode: string;
  nutrients: Nutrient[];
  nutrientToAdd: NutrientKindEnum;
  constructor() {
    this.title = "";
    this.brand = "";
    this.nutrients = [];
    this.barcode = "";
    this.barcode = "";
    this.nutrientToAdd = this.getLeftNutrients()[0];
    this.api = new DefaultApi();
    makeObservable(this, {
      title: observable,
      brand: observable,
      barcode: observable,
      nutrients: observable,
      updateTitle: action.bound,
      updateBrand: action.bound,
      updateBarcode: action.bound,
      addNutrient: action.bound,
      removeNutrient: action.bound,
      updateNutrient: action.bound,
      updateNutrientToAdd: action.bound,
    });
  }
  addNutrient() {
    this.nutrients.push({ kind: this.nutrientToAdd, percentage: 0 });
    this.nutrientToAdd = this.getLeftNutrients()[0];
  }
  removeNutrient(k: NutrientKindEnum) {
    this.nutrients = this.nutrients.filter((n) => n.kind !== k);
    this.nutrientToAdd = this.getLeftNutrients()[0];
  }
  updateNutrient(k: NutrientKindEnum, v: string) {
    const nutrient = this.nutrients.find((n) => n.kind === k);
    if (nutrient !== undefined) {
      nutrient.percentage = parseFloat(v) ?? 0;
    }
  }
  updateBarcode(title: string) {
    this.barcode = title;
  }
  updateBrand(title: string) {
    this.brand = title;
  }
  updateTitle(title: string) {
    this.title = title;
  }
  updateNutrientToAdd(nutrientToAdd: string) {
    this.nutrientToAdd = nutrientToAdd as NutrientKindEnum;
  }

  getLeftNutrients() {
    const currentTypes = this.nutrients.map((n) => n.kind);
    const allTypes = Object.values(NutrientKindEnum);
    const leftTypesSet = new Set(allTypes);
    for (const type of currentTypes) {
      leftTypesSet.delete(type);
    }

    return [...leftTypesSet];
  }

  async saveFoodItem() {
    if (this.brand !== "") {
      let fundamental: FundamentalFoodItem = {
        title: this.title,
        nutrients: this.nutrients,
      };
      let foodItemInner: FoodItemInner = {
        fundamental,
      };

      console.log("make api call");
      await this.api.createFoodItem({foodItemInner})
    }
  }
}

injectGlobal`
button,select {
  font-size: 18px;
}
input {
  width: 95%;
  font-size: 18px;
}
`;

const rootClass = css`
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  max-width: 400px;
  align-items: stretch;
`;
const titleClass = css`
  width: 30%;
  font-size: 25px;
  font-family: Arial;
`;

const doneButton = css`
  width: 100%;
`;
const textInput = css`
  width: 60%;
  font-size: 25px;
  font-family: Arial;
`;

const nutrientLabel = css`
  width: 30%;
  font-size: 25px;
  font-family: Arial;
`;
const nutrientValue = css`
  width: 30%;
`;
const nutrientRemove = css`
  width: 40%;
`;

export const placeNutrients = (state: FoodFormState) => {
  return (
    <>
      {state.nutrients.map((nutrient) => (
        <>
          <span className={nutrientLabel}> {nutrient.kind} </span>
          <span className={nutrientValue}>
            <input
              value={nutrient.percentage}
              type="number"
              onChange={(e) =>
                state.updateNutrient(nutrient.kind, e.target.value)
              }
            />
          </span>
          <span className={nutrientRemove}>
            <button onClick={(_) => state.removeNutrient(nutrient.kind)}>
              X
            </button>
          </span>
        </>
      ))}
    </>
  );
};

const newNutrientSelect = (state: FoodFormState) => {
  return (
    <select
      onChange={(e) => {
        state.updateNutrientToAdd(e.target.value);
      }}
    >
      {state.getLeftNutrients().map((type, ix) =>
        type === state.nutrientToAdd ? (
          <option selected value={type} key={ix}>
            {type}
          </option>
        ) : (
          <option value={type} key={ix}>
            {type}
          </option>
        )
      )}
    </select>
  );
};

const renderNutrientsTable = (state: FoodFormState) => {
  if (state.nutrientToAdd !== undefined) {
    return (
      <>
        <span className={titleClass}>{newNutrientSelect(state)}</span>
        <span className={textInput}>
          <button onClick={() => state.addNutrient()}>Add nutrient</button>
        </span>
      </>
    );
  } else {
    return <span />;
  }
};

export const NewFoodItemComponent = observer(
  ({ state }: { state: FoodFormState }) => {
    return (
      <div className={rootClass}>
        <span className={titleClass}> Title </span>
        <span className={textInput}>
          <input
            type="text"
            onChange={(e) => state.updateTitle(e.target.value)}
          />{" "}
        </span>

        <span className={titleClass}> Brand </span>
        <span className={textInput}>
          <input
            type="text"
            onChange={(e) => state.updateBrand(e.target.value)}
          />{" "}
        </span>
        <span className={titleClass}> Barcode </span>
        <span className={textInput}>
          <input
            type="text"
            onChange={(e) => state.updateBarcode(e.target.value)}
          />{" "}
        </span>

        {placeNutrients(state)}
        {renderNutrientsTable(state)}

        <span className={doneButton}>
          <button onClick={(_) => state.saveFoodItem()}> Done </button>
        </span>
      </div>
    );
  }
);
