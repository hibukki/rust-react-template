import { useState } from "react"
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom"
import { Layout } from "@/components/Layout"
import { LoginPage } from "@/pages/LoginPage"
import { ProfilePage } from "@/pages/ProfilePage"
import { ProfilesPage } from "@/pages/ProfilesPage"
import type { AuthResponse } from "@/api/client"
import "./index.css"

function App() {
  const [user, setUser] = useState<AuthResponse | null>(null)

  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout user={user} onLogout={() => setUser(null)} />}>
          <Route
            path="/login"
            element={
              user ? <Navigate to="/profiles" replace /> : <LoginPage onLogin={setUser} />
            }
          />
          <Route
            path="/profiles"
            element={user ? <ProfilesPage /> : <Navigate to="/login" replace />}
          />
          <Route
            path="/profile"
            element={
              user ? (
                <ProfilePage user={user} onUpdate={setUser} />
              ) : (
                <Navigate to="/login" replace />
              )
            }
          />
          <Route path="*" element={<Navigate to={user ? "/profiles" : "/login"} replace />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
