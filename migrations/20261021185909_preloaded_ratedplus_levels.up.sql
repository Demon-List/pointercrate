CREATE FUNCTION place_below (
    above INTEGER,
    demon_name TEXT,
    demon_requirement INTEGER,
    demon_video TEXT,
    demon_verifier INTEGER,
    demon_publisher INTEGER,
    demon_level_id BIGINT,
    demon_creators INTEGER[]
)
RETURNS void
LANGUAGE plpgsql
AS $$
DECLARE
    id INTEGER;
    target_position INTEGER;
    creator INTEGER;
    shifted TIMESTAMP := timezone('utc', clock_timestamp());
BEGIN
    IF above IS NULL THEN
        RETURN;
    END IF;

    SELECT position + 1 INTO target_position
    FROM demons
    WHERE demons.id = above;

    UPDATE demons SET position = position + 1 WHERE position >= target_position;

    INSERT INTO demons (name, position, requirement, video, verifier, publisher, level_id, rated)
    VALUES (demon_name, target_position, demon_requirement, demon_video, demon_verifier, demon_publisher, demon_level_id, 'f')
    RETURNING demons.id INTO id;

    FOREACH creator IN ARRAY demon_creators
    LOOP
        INSERT INTO creators (creator, demon) VALUES (creator, id);
    END LOOP;

    PERFORM recompute_rated_positions();
    PERFORM recompute_player_scores();
    PERFORM recompute_nation_scores();
    PERFORM recompute_subdivision_scores();
    REFRESH MATERIALIZED VIEW CONCURRENTLY player_ranks;

    UPDATE audit_log2 SET time = shifted WHERE time = timezone('utc', now());
END;
$$;

CREATE FUNCTION player(name TEXT)
RETURNS INTEGER
LANGUAGE sql
AS $$
    WITH ins AS (
        INSERT INTO players (name) SELECT name WHERE EXISTS (SELECT 1 FROM demons)
        ON CONFLICT (name) DO NOTHING RETURNING id
    )
    SELECT id FROM ins
    UNION ALL
    SELECT id FROM players WHERE players.name = player.name
    LIMIT 1;
$$;

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Prismline'),
    'BASSLINE JUNKIE',
    71,
    'https://www.youtube.com/watch?v=db_V19gW0rI',
    42337,
    player('MCBIGMACPLAYZ'),
    143300771,
    ARRAY[player('MCBIGMACPLAYZ')]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'ORDINARY'),
    'Marks',
    100,
    'https://www.youtube.com/watch?v=CxYXfCvNZtQ',
    51227,
    player('HazelSZL'),
    144895744,
    ARRAY[player('HazelSZL')]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Maniacal Chains'),
    'Thinking Space II',
    74,
    'https://www.youtube.com/watch?v=RJedeB2wHCA',
    6238,
    player('CoCyMain'),
    132854166,
    ARRAY[player('CoCyMain')]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Losing Your Mind'),
    'ETERNALtheory',
    100,
    'https://www.youtube.com/watch?v=3LfjQWk9Hyg',
    34530,
    34530,
    92196326,
    ARRAY[4504, 663, 37090]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'The Hallucination'),
    'Blooming Blossoms',
    77,
    'https://www.youtube.com/watch?v=GU-qfLRoDMo',
    34009,
    63719,
    130293163,
    ARRAY[63719]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Ashley Wave Trials'),
    'Old Orochi',
    60,
    'https://www.youtube.com/watch?v=YxtFAQtLYQY',
    65777,
    804,
    136742562,
    ARRAY[804]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Master of Puppets'),
    'Lost in Time 2',
    66,
    'https://www.youtube.com/watch?v=8i5GfTqb6oE',
    6238,
    6238,
    148297431,
    ARRAY[player('JOLA'), 55509, 71517, 4543, 58954, 59124, 55101, player('twigziy'), 6238, 54314, 34726, 41892, player('dawn'), player('BaimDaKing'), 51229, 41964, 42324, player('seuni'), 61450, 42546, player('skywyrd'), 37884]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'BPATA MPAKA'),
    'blight',
    100,
    'https://www.youtube.com/watch?v=2y1GVoPZjhU',
    55009,
    player('xSlendy'),
    146392813,
    ARRAY[player('xSlendy'), player('xKrohn'), 52104, player('Artoss'), player('M3tis'), 55496]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Through The Gates'),
    'Shadow Of The Past',
    76,
    'https://www.youtube.com/watch?v=LEyo0LJV8os',
    62241,
    57153,
    143909846,
    ARRAY[57153]
);

SELECT place_below(
    (SELECT id FROM demons WHERE name = 'Shadow Of The Past'),
    'ace',
    64,
    'https://www.youtube.com/watch?v=7uYghOdos8I',
    42612,
    player('TheBigCheese69'),
    138215607,
    ARRAY[player('TheBigCheese69'), player('TenshoSDK')]
);

DROP FUNCTION player;
DROP FUNCTION place_below;