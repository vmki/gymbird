import { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';
import { Exercise, API_BASE_URL } from '../data';
import styles from '../styles/ExerciseSelectionModal.module.css';

interface ExerciseSelectionModalProps {
  onSelect: (ex: Exercise) => void;
  onClose: () => void;
  show: boolean;
}

const ExerciseSelectionModal: React.FC<ExerciseSelectionModalProps> = ({ onSelect, show }) => {
  let [exercises, setExercises] = useState<Exercise[]>([]);

  const getExercises = () => {
    fetch(`${API_BASE_URL}/exercises`).then(res => res.json()).then(res => {
      console.log(JSON.parse(res));
      setExercises(JSON.parse(res));
    })
  }

  useEffect(() => {
    getExercises();
  }, [])

  let exerciseRender = exercises ?
    <div className={styles.modal}>
      <h1> Select Exercise</h1>
      <div className={styles.exercises}>
        { exercises.map((ex, idx) => (
          <button className={styles.exercise} key={idx} onClick={() => onSelect(ex) }>
            <img width={50} src={`assets/${ex.image_name}`} />
            <p>{ex.name}</p>
          </button>
        )) }
      </div>
    </div> 
    :
    <div className={styles.modal}>
      <h1>Loading...</h1>
    </div>

  const modal = show ?
    <div className={styles.modalOverlay}>
      { exerciseRender }
    </div> : null;

  if (typeof window !== "undefined") {
    return ReactDOM.createPortal(
      modal,
      document.getElementById("modal-root")
    );
  } else {
    return null;
  }
}
export default ExerciseSelectionModal;
