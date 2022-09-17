import ReactDOM from 'react-dom';
import { useState } from 'react';
import styles from '../styles/LoginModal.module.css';
import { LoginParameters, API_BASE_URL } from '../data';
import useStore from '../store';

export interface LoginModalProps {
  show: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

const LoginModal: React.FC<LoginModalProps> = ({ show, onClose, onSuccess }: LoginModalProps) => {
  let [showPassword, setShowPassword] = useState(false);

  let [password, setPassword] = useState("");
  let [email, setEmail] = useState("");
  let [error, setError] = useState<string | null>(null);

  let store: any = useStore();

  const login = async (p: LoginParameters) => {
    return fetch(`${API_BASE_URL}/login`, {
      method: "POST",
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(p),
    }).then(res => res.json()).then(res => {
      let data = JSON.parse(res);

      if(data.error) {
        setError(data.error);
        return;
      }

      store.setToken(data.session_token);

      onSuccess();
    })
  }


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
          onClick={() => login({ email: email, password: password })}
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
