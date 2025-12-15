import { useState, useEffect, useCallback } from "react";
import { api, type AuthResponse } from "./api/client";
import { useWebSocket } from "./hooks/useWebSocket";
import type { Profile } from "./types/bindings";
import "./App.css";

function App() {
  const [user, setUser] = useState<AuthResponse | null>(null);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [error, setError] = useState<string | null>(null);

  // Form state
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [editBio, setEditBio] = useState("");
  const [isRegistering, setIsRegistering] = useState(false);

  // WebSocket handlers
  const handleProfileCreated = useCallback((profile: Profile) => {
    setProfiles((prev) => [profile, ...prev]);
  }, []);

  const handleProfileUpdated = useCallback((profile: Profile) => {
    setProfiles((prev) =>
      prev.map((p) => (p.id === profile.id ? profile : p))
    );
  }, []);

  const { isConnected } = useWebSocket({
    onProfileCreated: handleProfileCreated,
    onProfileUpdated: handleProfileUpdated,
  });

  // Load profiles on mount
  useEffect(() => {
    api.profiles
      .list()
      .then(setProfiles)
      .catch((e) => setError(e.message));
  }, []);

  const handleAuth = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    try {
      if (isRegistering) {
        const response = await api.auth.register({
          email,
          password,
          display_name: displayName,
        });
        setUser(response);
      } else {
        const response = await api.auth.login({ email, password });
        setUser(response);
      }
      setEmail("");
      setPassword("");
      setDisplayName("");
    } catch (e) {
      setError((e as Error).message);
    }
  };

  const handleLogout = async () => {
    try {
      await api.auth.logout();
      setUser(null);
    } catch (e) {
      setError((e as Error).message);
    }
  };

  const handleUpdateProfile = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!user) return;
    setError(null);
    try {
      const updated = await api.profiles.update(user.profile.id, {
        bio: editBio || undefined,
      });
      setUser({ ...user, profile: updated });
      setEditBio("");
    } catch (e) {
      setError((e as Error).message);
    }
  };

  return (
    <div className="app">
      <header>
        <h1>Profiles</h1>
        <span className={`status ${isConnected ? "connected" : "disconnected"}`}>
          {isConnected ? "Live" : "Reconnecting..."}
        </span>
      </header>

      {error && <div className="error">{error}</div>}

      {!user ? (
        <form onSubmit={handleAuth} className="auth-form">
          <h2>{isRegistering ? "Register" : "Login"}</h2>
          <input
            type="email"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
          <input
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
          {isRegistering && (
            <input
              type="text"
              placeholder="Display Name"
              value={displayName}
              onChange={(e) => setDisplayName(e.target.value)}
              required
            />
          )}
          <button type="submit">{isRegistering ? "Register" : "Login"}</button>
          <button type="button" onClick={() => setIsRegistering(!isRegistering)}>
            {isRegistering ? "Have an account? Login" : "Need an account? Register"}
          </button>
        </form>
      ) : (
        <div className="user-section">
          <div className="user-info">
            <p>Logged in as: {user.profile.display_name}</p>
            <button onClick={handleLogout}>Logout</button>
          </div>
          <form onSubmit={handleUpdateProfile} className="edit-form">
            <h3>Update Your Profile</h3>
            <textarea
              placeholder="Bio"
              value={editBio}
              onChange={(e) => setEditBio(e.target.value)}
            />
            <button type="submit">Update</button>
          </form>
        </div>
      )}

      <section className="profiles">
        <h2>All Profiles</h2>
        {profiles.length === 0 ? (
          <p>No profiles yet</p>
        ) : (
          <ul>
            {profiles.map((profile) => (
              <li key={profile.id.toString()}>
                <strong>{profile.display_name}</strong>
                {profile.bio && <p>{profile.bio}</p>}
                <small>Updated: {profile.updated_at}</small>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}

export default App;
