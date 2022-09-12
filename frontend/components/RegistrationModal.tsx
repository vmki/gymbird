import ReactDOM from 'react-dom';
import { useState } from 'react';
import styles from '../styles/RegistrationModal.module.css';
import { RegistrationParameters } from '../types';

export interface RegistrationModalProps {
  show: boolean;
  onClose: () => void;
  onSubmit: (p: RegistrationParameters) => void;
}

const RegistrationModal: React.FC<RegistrationModalProps> = ({ show, onClose, onSubmit }: RegistrationModalProps) => {
  let [showPassword, setShowPassword] = useState(false);
  let [password, setPassword] = useState("");
  let [email, setEmail] = useState("");
  let [name, setName] = useState("");
  let [username, setUsername] = useState("");

  const modal = show && typeof window !== "undefined" ?
    <div className={styles.modalOverlay}>
      <div className={styles.modal}>
        <h1>Register</h1>

        <input
          onChange={(e: any) => setName(e.target.value)}
          type="text"
          name="name"
          placeholder="Name"
        />

        <input
          onChange={(e: any) => setUsername(e.target.value)}
          type="text"
          name="username"
          placeholder="Username"
        />

        <input
          onChange={(e: any) => setEmail(e.target.value)}
          type="email"
          name="name"
          placeholder="Email"
        />

        <input
          onChange={(e: any) => setPassword(e.target.value)}
          type={showPassword ? "text" : "password"}
          name="name"
          placeholder="Password"
        />

        <button
          className={styles.showPasswordButton}
          onClick={() => setShowPassword(!showPassword) }
        >{showPassword ? "Hide password" : "Show Password"}</button>

        <button
          onClick={() => onSubmit({ username: username, name: name, email: email, password: password })}
          className={styles.loginButton}
        >Register</button>
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

export default RegistrationModal;
