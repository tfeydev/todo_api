using Pkg
Pkg.activate(@__DIR__)

using HTTP
using JSON3

"""
    calculate_score(title::String)::Float64

Calculates a complexity score for a new To-Do item based on its title.
"""
function calculate_score(title::String)::Float64
    score = 0.0

    # 1. Base score based on title length (longer = more complex)
    score += length(title) * 0.1

    # 2. Bonus for specific keywords
    if occursin("Refactor", title) || occursin("refactor", title)
        score += 5.0
    end

    if occursin("Urgent", title) || occursin("urgent", title)
        score += 2.0
    end

    return round(score, digits=2)
end


# === HTTP SERVER ===
const ROUTER = HTTP.Router()

# Friendly GET info page for browser visits
# FIX: Added 'charset=utf-8' to prevent browser encoding warnings.
HTTP.register!(ROUTER, "GET", "/score", req -> begin
    msg = """
    ✅ Julia Task Scorer Service
    ---------------------------
    This endpoint calculates a complexity score for To-Do titles.

    Use POST /score with JSON data like:
        {"title": "Refactor backend API"}

    Example:
        curl -X POST http://127.0.0.1:8081/score \\
             -H "Content-Type: application/json" \\
             -d '{"title": "Urgent: Refactor backend API"}'

    You will get:
        {"score": 6.2}
    """
    # Response headers now include charset for text/plain
    return HTTP.Response(200, ["Content-Type" => "text/plain; charset=utf-8"], msg)
end)

# POST handler (the real logic)
HTTP.register!(ROUTER, "POST", "/score", req -> begin
    if HTTP.hasheader(req, "Content-Type") &&
       HTTP.header(req, "Content-Type") == "application/json"

        try
            data = JSON3.read(IOBuffer(HTTP.payload(req)))
            title = String(data[:title])
            score = calculate_score(title)
            response_body = JSON3.write((score=score,))
            return HTTP.Response(200, ["Content-Type" => "application/json"], body=response_body)

        catch e
            @warn "Failed to parse JSON or process request: $e"
            return HTTP.Response(400, "Invalid request format.")
        end
    else
        return HTTP.Response(415, "Unsupported Media Type, expected application/json.")
    end
end)

println("✅ Julia Task Scorer Server started at http://127.0.0.1:8081")
HTTP.serve(ROUTER, "127.0.0.1", 8081)
