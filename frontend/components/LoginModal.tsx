import ReactDOM from 'react-dom';
import { useState } from 'react';
import styles from '../styles/LoginModal.module.css';
import { LoginParameters } from '../data';

export interface LoginModalProps {
  show: boolean;
  onClose: () => void;
  onSubmit: (params: LoginParameters) => void;
}

const LoginModal: React.FC<LoginModalProps> = ({ show, onClose, onSubmit }: LoginModalProps) => {
  let [showPassword, setShowPassword] = useState(false);

  let [password, setPassword] = useState("");
  let [email, setEmail] = useState("");

  const modal = show ?
    <div className={styles.modalOverlay}>
      <div className={styles.modal}>
        <h1>Log in</h1>

        <input
          onChange={(e: any) => setEmail(e.target.value)}
          type="email"
          name="name"
          placeholder="Email"
        />

        <input
          onChange={(e: any) => setPassword(e.target.value) }
          type={showPassword ? "text" : "password"}
          name="name"
          placeholder="Password"
        />

        <button
          className={styles.showPasswordButton}
          onClick={() => setShowPassword(!showPassword) }
        >{showPassword ? "Hide password" : "Show Password"}</button>

        <button
          onClick={() => onSubmit({ email: email, password: password })}
          className={styles.loginButton}
        >Log in</button>
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
