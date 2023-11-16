select round(
                   cast(
                                   (SELECT count(distinct Activity.player_id)
                                    FROM Activity
                                             LEFT JOIN Activity b ON Activity.player_id = b.player_id AND
                                                                     b.event_date = DATEADD(day, 1, Activity.event_date)

                                    WHERE NOT EXISTS (select 1
                                                      FROM Activity t2
                                                      WHERE t2.player_id = Activity.player_id
                                                        AND t2.event_date < Activity.event_date)
                                      AND b.event_date IS NOT NULL) * 1000
                               / (SELECT COUNT(DISTINCT player_id) FROM Activity)
                       as float) / 1000
           , 2)
           AS fraction