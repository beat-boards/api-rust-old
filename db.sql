--
-- PostgreSQL database dump
--

-- Dumped from database version 11.4
-- Dumped by pg_dump version 11.4

-- Started on 2019-08-03 03:02:48

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 2 (class 3079 OID 16597)
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- TOC entry 617 (class 1247 OID 16555)
-- Name: difficulty; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.difficulty AS ENUM (
    'easy',
    'normal',
    'hard',
    'expert',
    'expert_plus'
);


--
-- TOC entry 625 (class 1247 OID 16612)
-- Name: modifier; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.modifier AS ENUM (
    'disappearing_arrows',
    'faster_song',
    'ghost_notes',
    'no_arrows',
    'no_bombs',
    'no_fail',
    'no_obstacles',
    'slower_song'
);


--
-- TOC entry 621 (class 1247 OID 16573)
-- Name: role; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.role AS ENUM (
    'owner',
    'contributor',
    'supporter',
    'ranker',
    'curator',
    'score_saber',
    'player',
    'toxic'
);


--
-- TOC entry 202 (class 1255 OID 16535)
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


--
-- TOC entry 201 (class 1255 OID 16536)
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


SET default_with_oids = false;

--
-- TOC entry 200 (class 1259 OID 16537)
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


--
-- TOC entry 198 (class 1259 OID 16443)
-- Name: maps; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.maps (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    hash text NOT NULL,
    difficulty public.difficulty NOT NULL,
    song_name text NOT NULL,
    song_sub_name text NOT NULL,
    song_author_name text NOT NULL,
    level_author_name text NOT NULL,
    difficulty_rating double precision NOT NULL,
    length double precision NOT NULL,
    bpm double precision NOT NULL,
    note_jump_speed double precision NOT NULL,
    note_count integer NOT NULL,
    complexity double precision NOT NULL,
    saber_distance double precision NOT NULL,
    max_rp double precision NOT NULL,
    upvotes integer DEFAULT 0 NOT NULL,
    downvotes integer DEFAULT 0 NOT NULL
);


--
-- TOC entry 199 (class 1259 OID 16508)
-- Name: scores; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.scores (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    "user" uuid NOT NULL,
    map uuid NOT NULL,
    date timestamp without time zone NOT NULL,
    raw_score integer NOT NULL,
    raw_percentage double precision NOT NULL,
    modifiers public.modifier[] NOT NULL,
    adjusted_score integer NOT NULL,
    raw_rp double precision NOT NULL,
    adjusted_rp double precision NOT NULL
);


--
-- TOC entry 197 (class 1259 OID 16429)
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    steam_id bigint,
    oculus_id text,
    banned boolean DEFAULT false NOT NULL,
    username text NOT NULL,
    role public.role DEFAULT 'player'::public.role NOT NULL,
    country text NOT NULL,
    rp double precision DEFAULT 0.0 NOT NULL,
    fails integer DEFAULT 0 NOT NULL,
    following uuid[] DEFAULT ARRAY[]::uuid[] NOT NULL,
    image text
);


--
-- TOC entry 2740 (class 2606 OID 16542)
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- TOC entry 2734 (class 2606 OID 16450)
-- Name: maps map_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.maps
    ADD CONSTRAINT map_pkey PRIMARY KEY (id);


--
-- TOC entry 2738 (class 2606 OID 16515)
-- Name: scores score_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scores
    ADD CONSTRAINT score_pkey PRIMARY KEY (id);


--
-- TOC entry 2732 (class 2606 OID 16436)
-- Name: users user_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- TOC entry 2735 (class 1259 OID 16527)
-- Name: fki_map_fkey; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX fki_map_fkey ON public.scores USING btree (map);


--
-- TOC entry 2736 (class 1259 OID 16521)
-- Name: fki_user_fkey; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX fki_user_fkey ON public.scores USING btree ("user");


--
-- TOC entry 2742 (class 2606 OID 16522)
-- Name: scores map_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scores
    ADD CONSTRAINT map_fkey FOREIGN KEY (map) REFERENCES public.maps(id);


--
-- TOC entry 2741 (class 2606 OID 16516)
-- Name: scores user_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scores
    ADD CONSTRAINT user_fkey FOREIGN KEY ("user") REFERENCES public.users(id);


-- Completed on 2019-08-03 03:02:48

--
-- PostgreSQL database dump complete
--

