export interface JWTPayload {
    id: number
    sub: string
    exp: number
}

export interface AuthResponse {
    token: string
    payload: JWTPayload
}