import { useState, useCallback } from "react"
import { useWebSocket } from "@/hooks/useWebSocket"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import type { Profile } from "@/types/bindings"

export function ProfilesPage() {
  const [profiles, setProfiles] = useState<Map<bigint, Profile>>(new Map())

  // Single handler for all profile events (initial state AND updates)
  const handleProfile = useCallback((profile: Profile) => {
    setProfiles((prev) => new Map(prev).set(profile.id, profile))
  }, [])

  const { isConnected } = useWebSocket({
    onProfile: handleProfile,
  })

  const profileList = Array.from(profiles.values())

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold">All Profiles</h1>
          <p className="text-muted-foreground">Browse profiles from all users</p>
        </div>
        <div className="flex items-center gap-2">
          <span
            className={`h-2 w-2 rounded-full ${isConnected ? "bg-green-500" : "bg-yellow-500"}`}
          />
          <span className="text-sm text-muted-foreground">
            {isConnected ? "Live" : "Connecting..."}
          </span>
        </div>
      </div>

      {!isConnected && profiles.size === 0 ? (
        <div className="text-center text-muted-foreground">Connecting...</div>
      ) : profileList.length === 0 ? (
        <Card>
          <CardContent className="py-8 text-center text-muted-foreground">
            No profiles yet. Be the first to create one!
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {profileList.map((profile) => (
            <Card key={profile.id.toString()}>
              <CardHeader>
                <CardTitle className="text-lg">{profile.display_name}</CardTitle>
                <CardDescription>
                  Updated {new Date(profile.updated_at).toLocaleDateString()}
                </CardDescription>
              </CardHeader>
              {profile.bio && (
                <CardContent>
                  <p className="text-sm text-muted-foreground">{profile.bio}</p>
                </CardContent>
              )}
            </Card>
          ))}
        </div>
      )}
    </div>
  )
}
