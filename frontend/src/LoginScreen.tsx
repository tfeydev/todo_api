// src/LoginScreen.tsx

import React, { useState } from 'react';
import { useAuth } from './AuthContext';

const LoginScreen: React.FC = () => {
    const { login, authError } = useAuth();
    const [email, setEmail] = useState("thor@techthor.com");
    const [password, setPassword] = useState("secret123");
    const [loading, setLoading] = useState(false);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setLoading(true);
        try {
            await login(email, password);
        } catch (e) {
            // Error is already stored and displayed by the context
            console.log("Login attempt failed.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="bg-gray-50 dark:bg-gray-800 p-6 rounded-lg shadow max-w-sm mx-auto mt-20">
            <h2 className="text-xl font-semibold mb-4 text-center">Login Required</h2>
            <form onSubmit={handleSubmit}>
                <input
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="E-Mail"
                    className="w-full mb-3 border px-3 py-2 rounded bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                />
                <input
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    placeholder="Passwort"
                    className="w-full mb-4 border px-3 py-2 rounded bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                />
                <button
                    type="submit"
                    className="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition disabled:bg-blue-400"
                    disabled={loading || !email || !password}
                >
                    {loading ? "Logging in..." : "Login"}
                </button>
            </form>
            {authError && <p className="text-red-500 text-sm mt-3 text-center">{authError}</p>}
        </div>
    );
};

export default LoginScreen;