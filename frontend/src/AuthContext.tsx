// src/AuthContext.tsx

import React, { createContext, useContext, useState, useEffect } from 'react';
import type { ReactNode } from 'react';

// --- Types & Constants ---
const TOKEN_KEY = "jwt_token";
const DEFAULT_AUTH_URL = "http://localhost:3000/auth/login";

// HeadersInit is a global type and MUST NOT be imported from 'react' or 'node-fetch'
interface AuthContextType {
    token: string | null;
    login: (email: string, password: string) => Promise<void>;
    logout: () => void;
    authError: string;
    getAuthHeaders: () => HeadersInit; 
    isReady: boolean; // CRITICAL: Indicates if initial localStorage check is done
}

// Initial Context value
const AuthContext = createContext<AuthContextType | undefined>(undefined);

// Hook to consume the Context
export const useAuth = () => {
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider');
    }
    return context;
};

// --- Provider Component ---
export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [token, setToken] = useState<string | null>(null);
    const [authError, setAuthError] = useState("");
    const [isReady, setIsReady] = useState(false); 

    // Effect to load token from localStorage once on mount
    useEffect(() => {
        const storedToken = localStorage.getItem(TOKEN_KEY);
        if (storedToken) {
            setToken(storedToken);
        }
        setIsReady(true); // Set to true after initial check (even if token is null)
    }, []);

    // Login logic
    const login = async (email: string, password: string) => {
        setAuthError("");
        // ... (rest of the login logic remains the same)
        try {
            const res = await fetch(DEFAULT_AUTH_URL, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email, password }),
            });

            if (!res.ok) {
                const errorData = await res.json();
                setAuthError(errorData.error || "Login failed");
                throw new Error("Login failed");
            }

            const data = await res.json();
            const newToken = data.token;
            
            setToken(newToken);
            localStorage.setItem(TOKEN_KEY, newToken);
        } catch (error) {
            setAuthError("Failed to connect or log in.");
            console.error(error);
            throw error;
        }
    };

    // Logout logic
    const logout = () => {
        setToken(null);
        localStorage.removeItem(TOKEN_KEY);
    };

    // Header helper: Always includes Content-Type, includes Authorization if token exists
    const getAuthHeaders = (): HeadersInit => {
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
        };

        if (token) {
            headers["Authorization"] = `Bearer ${token}`;
        }
        
        return headers; 
    };

    const value = {
        token,
        login,
        logout,
        authError,
        getAuthHeaders,
        isReady,
    };

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};