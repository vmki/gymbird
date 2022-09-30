export const API_BASE_URL: string = "http://localhost:8080/api";

export type SetUserFunction = (value: User | null) => void;

export enum SetMeasurement {
  Weight,

  // In seconds.
  Length,

  Reps
}

export interface Store {
  sessionToken: string,
  setToken: (value: string | null) => void;
}

export interface UtilityParameters {
  store: Store;
  setUser: SetUserFunction;
}

export interface LoginParameters {
  email: string;
  password: string;
}

export interface Exercise {
  id: string;
  name: string;
  image_name: string;
}

export interface RegistrationParameters {
  email: string;
  username: string;
  name: string;
  password: string;
}

export interface User {
  username: string;
  password: string;
  email: string;
  name: string;
  id: string;
}

/// SetType enum and stuff like WeightSet, RepSet
export interface Set {
  reps: number;
  weight: number;
}

export interface WorkoutExercise {
  sets: Set[]
  exercise: Exercise
}
