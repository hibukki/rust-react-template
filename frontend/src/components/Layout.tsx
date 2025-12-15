import { NavLink, Outlet, useNavigate } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { api, type AuthResponse } from "@/api/client"
import { cn } from "@/lib/utils"

interface LayoutProps {
  user: AuthResponse | null
  onLogout: () => void
}

export function Layout({ user, onLogout }: LayoutProps) {
  const navigate = useNavigate()

  const handleLogout = async () => {
    try {
      await api.auth.logout()
    } catch {
      // Ignore logout errors
    }
    onLogout()
    navigate("/login")
  }

  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="mx-auto flex h-14 max-w-4xl items-center justify-between px-4">
          <nav className="flex items-center gap-6">
            <span className="font-semibold">Profiles App</span>
            {user && (
              <div className="flex items-center gap-1">
                <NavLink
                  to="/profiles"
                  className={({ isActive }) =>
                    cn(
                      "px-3 py-1.5 text-sm font-medium rounded-md transition-colors",
                      isActive
                        ? "bg-secondary text-secondary-foreground"
                        : "text-muted-foreground hover:text-foreground hover:bg-secondary/50"
                    )
                  }
                >
                  All Profiles
                </NavLink>
                <NavLink
                  to="/profile"
                  className={({ isActive }) =>
                    cn(
                      "px-3 py-1.5 text-sm font-medium rounded-md transition-colors",
                      isActive
                        ? "bg-secondary text-secondary-foreground"
                        : "text-muted-foreground hover:text-foreground hover:bg-secondary/50"
                    )
                  }
                >
                  Your Profile
                </NavLink>
              </div>
            )}
          </nav>
          {user && (
            <div className="flex items-center gap-4">
              <span className="text-sm text-muted-foreground">{user.profile.display_name}</span>
              <Button variant="outline" size="sm" onClick={handleLogout}>
                Sign out
              </Button>
            </div>
          )}
        </div>
      </header>
      <main className="mx-auto max-w-4xl p-4">
        <Outlet />
      </main>
    </div>
  )
}
