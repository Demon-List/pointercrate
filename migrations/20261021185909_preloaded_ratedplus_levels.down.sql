SET CONSTRAINTS unique_position DEFERRED;

ALTER TABLE creators DISABLE TRIGGER USER;
ALTER TABLE demons DISABLE TRIGGER USER;
ALTER TABLE players DISABLE TRIGGER USER;

DELETE FROM audit_log2 WHERE time IN (SELECT time FROM demon_additions WHERE id IN (SELECT id FROM demons WHERE rated = 'f'));
DELETE FROM demon_modifications WHERE id IN (SELECT id FROM demons WHERE rated = 'f');
DELETE FROM player_modifications WHERE id IN (SELECT id FROM players WHERE name IN ('Artoss','BaimDaKing','CoCyMain','dawn','HazelSZL','JOLA','M3tis','MCBIGMACPLAYZ','seuni','skywyrd','TenshoSDK','TheBigCheese69','twigziy','xKrohn','xSlendy'));

DELETE FROM creators WHERE demon IN (SELECT id FROM demons WHERE rated = 'f');
DELETE FROM demons WHERE rated = 'f';
DELETE FROM players WHERE name IN ('Artoss','BaimDaKing','CoCyMain','dawn','HazelSZL','JOLA','M3tis','MCBIGMACPLAYZ','seuni','skywyrd','TenshoSDK','TheBigCheese69','twigziy','xKrohn','xSlendy');

UPDATE demons SET position = d.rownumber
FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY position) AS rownumber FROM demons) d
WHERE demons.id = d.id;

SELECT recompute_rated_positions();
SELECT recompute_player_scores();
SELECT recompute_nation_scores();
SELECT recompute_subdivision_scores();
REFRESH MATERIALIZED VIEW CONCURRENTLY player_ranks;

ALTER TABLE creators ENABLE TRIGGER USER;
ALTER TABLE demons ENABLE TRIGGER USER;
ALTER TABLE players ENABLE TRIGGER USER;
