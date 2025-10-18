// src/App.tsx

import React, { useState, useEffect } from "react";
import { AuthProvider, useAuth } from './AuthContext';
import LoginScreen from './LoginScreen';
import TodoList from './TodoList';

// Initialize Dark Mode State synchronized with system preference or localStorage
const getInitialDark = () => {
    const saved = localStorage.getItem("theme");
    if (saved === "dark") return true;
    if (saved === "light") return false;
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
};


// --- Main App Logic (Component with access to Auth Context) ---
const MainAppContent: React.FC = () => {
    const { token } = useAuth();
    const [isDark, setIsDark] = useState(getInitialDark);

    // Dark Mode effect: applies class to HTML element and saves preference
    useEffect(() => {
        document.documentElement.classList.toggle("dark", isDark);
        localStorage.setItem("theme", isDark ? "dark" : "light");
    }, [isDark]);

    const toggleDark = () => setIsDark(prev => !prev);


    return (
        <div className="min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 font-sans px-4">
            {token ? (
                // Authenticated: Show the Todo List
                <TodoList isDark={isDark} toggleDark={toggleDark} />
            ) : (
                // Not authenticated: Show the Login Page
                <LoginScreen />
            )}
        </div>
    );
};


// --- AuthProvider Wrapper ---
function App() {
    return (
        <AuthProvider>
            <MainAppContent />
        </AuthProvider>
    );
}

export default App;