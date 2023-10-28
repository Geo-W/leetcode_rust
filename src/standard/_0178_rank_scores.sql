select score, DENSE_RANK() OVER (ORDER BY score desc) AS rank
FROM Scores
order by score desc
