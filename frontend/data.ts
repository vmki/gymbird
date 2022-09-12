export const API_BASE_URL: string = "http://localhost:8080/api";

export interface LoginParameters {
  email: string;
  password: string;
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
