import create from 'zustand';
import { persist} from 'zustand/middleware';

const useStore = create(
  persist(
    set => ({
      sessionToken: null,

      setToken: (token: string) => {
        set((_: any) => ({
          sessionToken: token,
        }))
      }
    })
  )
);


export default useStore;
