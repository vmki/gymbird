import ReactDOM from 'react-dom';
import styles from '../styles/LoginModal.module.css';

export interface LoginModalProps {
  show: boolean;
  onClose: () => void;
}

const LoginModal: React.FC<LoginModalProps> = ({ show }: LoginModalProps) => {
  const modal = show ?
    <div className={styles.modalOverlay}>
      <div className={styles.modal}>
        <h1>Hello</h1>
      </div>
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

export default LoginModal;
