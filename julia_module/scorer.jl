# julia_module/scorer.jl

using HTTP
using JSON3

"""
    calculate_score(title::String)::Float64

Calculates a complexity score for a new To-Do item based on its title.
"""
function calculate_score(title::String)::Float64
    score = 0.0

    # 1. Base score based on title length (longer tasks are generally more complex)
    score += length(title) * 0.1 

    # 2. Bonus score for specific keywords
    if occursin("Refactor", title) || occursin("refactor", title)
        score += 5.0 # Large bonus for heavy keywords
    end

    if occursin("Urgent", title) || occursin("urgent", title)
        score += 2.0 # Smaller bonus for urgency
    end

    return round(score, digits=2)
end


# HTTP handler to process the POST request
const ROUTER = HTTP.Router()

HTTP.register!(ROUTER, "POST", "/score", req -> begin
    # Check for correct Content-Type
    if HTTP.hasheader(req, "Content-Type" => "application/json")
        try
            # Parse JSON body
            data = JSON3.read(IOBuffer(HTTP.payload(req)))
            title = String(data[:title])
            
            # Calculate and round the score
            score = calculate_score(title)

            # Send back JSON response
            response_body = JSON3.write((score=score,))
            return HTTP.Response(200, ["Content-Type" => "application/json"], body=response_body)

        catch e
            # Error handling for invalid JSON or processing issues
            @warn "Failed to parse JSON or process request: $e"
            return HTTP.Response(400, "Invalid request format.")
        end
    else
        return HTTP.Response(415, "Unsupported Media Type, expected application/json.")
    end
end)

println("✅ Julia Task Scorer Server started at http://127.0.0.1:8081")
HTTP.serve(ROUTER, "127.0.0.1", 8081)