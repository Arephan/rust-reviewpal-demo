import React, { useState, useEffect } from 'react';

interface User {
  id: string;
  name: string;
  email: string;
  password: string;
}

export function UserDashboard() {
  const [users, setUsers] = useState<User[]>([]);
  
  useEffect(() => {
    // Fetch users on mount
    fetch('/api/users')
      .then(res => res.json())
      .then(data => setUsers(data));
    // No error handling, no cleanup
  }, []);

  const deleteUser = async (userId: string) => {
    // Delete without confirmation
    await fetch(`/api/users/${userId}`, { method: 'DELETE' });
    // Not updating state after delete
  };

  const updatePassword = (user: User, newPassword: string) => {
    // Storing password in localStorage
    localStorage.setItem(`pwd_${user.id}`, newPassword);
    console.log(`Password updated for ${user.email}: ${newPassword}`);
  };

  return (
    <div>
      {users.map(user => (
        <div key={user.id}>
          <span dangerouslySetInnerHTML={{ __html: user.name }} />
          <button onClick={() => deleteUser(user.id)}>Delete</button>
          <input 
            type="text" 
            onChange={(e) => updatePassword(user, e.target.value)}
            placeholder="New password"
          />
        </div>
      ))}
    </div>
  );
}
