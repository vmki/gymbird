import { NextPage } from 'next';
import { API_BASE_URL } from '../data';
import { useEffect, useState } from 'react';
import Exercise from '../components/Exercise';
import styles from '../styles/Exercises.module.css';

const Exercises: NextPage = () => {
  let [exercises, setExercises] = useState<any[]>([]);

  const getExercises = () => {
    fetch(`${API_BASE_URL}/exercises`).then(res => res.json()).then(res => {
      console.log(JSON.parse(res));
      setExercises(JSON.parse(res));
    })
  }

  useEffect(() => {
    getExercises();
  }, [])

  let exercisesRender = exercises ?
       exercises.sort((a: any, b: any) => a.name > b.name).map(ex => (
          <Exercise name={ex.name} image_name={ex.image_name} id={ex.id} />
      )) : null;
  
  return (
    <div className={styles.exerciseContainer}>
      { exercisesRender }
    </div>
  )
}

export default Exercises;
