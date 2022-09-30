import { User, API_BASE_URL, SetUserFunction, UtilityParameters, Exercise } from './data';

export const fetchUser = async (params: UtilityParameters) => {
  if(params.store.sessionToken !== null) {
    fetch(`${API_BASE_URL}/user`, {
      headers: {
        'Authorization': params.store.sessionToken,
      }
    }).then(res => res.json()).then(res => params.setUser(JSON.parse(res)));
  }
}

export const getExercise = async (id: string): Promise<Exercise> => {
  let response = await fetch(`${API_BASE_URL}/exercise/${id}`);

  console.log(await response.json());
  return await response.json();
}

export const logOut = async(params: UtilityParameters) => {
  if(params.store.sessionToken !== null) {
    await fetch(`${API_BASE_URL}/logout`, {
      headers: {
        'Authorization': params.store.sessionToken,
      }
    })

    params.store.setToken(null);
    params.setUser(null);
  }
}
