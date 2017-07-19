--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.3
-- Dumped by pg_dump version 9.6.3

-- Started on 2017-07-12 13:31:10 CEST

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 6 (class 2615 OID 16388)
-- Name: lopeningent; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA lopeningent;


ALTER SCHEMA lopeningent OWNER TO postgres;

--
-- TOC entry 1 (class 3079 OID 12393)
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- TOC entry 2175 (class 0 OID 0)
-- Dependencies: 1
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET search_path = lopeningent, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 190 (class 1259 OID 16408)
-- Name: edge_nodes; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE edge_nodes (
    eid integer NOT NULL,
    nid integer NOT NULL
);


ALTER TABLE edge_nodes OWNER TO postgres;

--
-- TOC entry 189 (class 1259 OID 16399)
-- Name: edges; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE edges (
    eid integer NOT NULL,
    rating real,
    tags character varying(128)[],
    CONSTRAINT rating_cap CHECK (((rating >= (0.0)::double precision) AND (rating <= (5.0)::double precision)))
);


ALTER TABLE edges OWNER TO postgres;

--
-- TOC entry 188 (class 1259 OID 16397)
-- Name: edges_eid_seq; Type: SEQUENCE; Schema: lopeningent; Owner: postgres
--

CREATE SEQUENCE edges_eid_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE edges_eid_seq OWNER TO postgres;

--
-- TOC entry 2176 (class 0 OID 0)
-- Dependencies: 188
-- Name: edges_eid_seq; Type: SEQUENCE OWNED BY; Schema: lopeningent; Owner: postgres
--

ALTER SEQUENCE edges_eid_seq OWNED BY edges.eid;


--
-- TOC entry 187 (class 1259 OID 16391)
-- Name: nodes; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE nodes (
    nid integer NOT NULL,
    coord point
);


ALTER TABLE nodes OWNER TO postgres;

--
-- TOC entry 186 (class 1259 OID 16389)
-- Name: nodes_nid_seq; Type: SEQUENCE; Schema: lopeningent; Owner: postgres
--

CREATE SEQUENCE nodes_nid_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE nodes_nid_seq OWNER TO postgres;

--
-- TOC entry 2177 (class 0 OID 0)
-- Dependencies: 186
-- Name: nodes_nid_seq; Type: SEQUENCE OWNED BY; Schema: lopeningent; Owner: postgres
--

ALTER SEQUENCE nodes_nid_seq OWNED BY nodes.nid;


--
-- TOC entry 192 (class 1259 OID 16425)
-- Name: pois; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE pois (
    pid integer NOT NULL,
    name character varying(128),
    description text,
    coord point,
    type character varying(128)
);


ALTER TABLE pois OWNER TO postgres;

--
-- TOC entry 191 (class 1259 OID 16423)
-- Name: pois_pid_seq; Type: SEQUENCE; Schema: lopeningent; Owner: postgres
--

CREATE SEQUENCE pois_pid_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE pois_pid_seq OWNER TO postgres;

--
-- TOC entry 2178 (class 0 OID 0)
-- Dependencies: 191
-- Name: pois_pid_seq; Type: SEQUENCE OWNED BY; Schema: lopeningent; Owner: postgres
--

ALTER SEQUENCE pois_pid_seq OWNED BY pois.pid;


--
-- TOC entry 194 (class 1259 OID 16437)
-- Name: routes; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE routes (
    rid integer NOT NULL,
    edges_list integer[],
    route_rating smallint,
    time_requested timestamp with time zone
);


ALTER TABLE routes OWNER TO postgres;

--
-- TOC entry 193 (class 1259 OID 16435)
-- Name: routes_rid_seq; Type: SEQUENCE; Schema: lopeningent; Owner: postgres
--

CREATE SEQUENCE routes_rid_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE routes_rid_seq OWNER TO postgres;

--
-- TOC entry 2179 (class 0 OID 0)
-- Dependencies: 193
-- Name: routes_rid_seq; Type: SEQUENCE OWNED BY; Schema: lopeningent; Owner: postgres
--

ALTER SEQUENCE routes_rid_seq OWNED BY routes.rid;


--
-- TOC entry 195 (class 1259 OID 16446)
-- Name: users; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE users (
    uid integer NOT NULL,
    avg_speed real,
    avg_heartrate integer,
    avg_distance integer,
    tot_distance integer,
    tot_duration abstime,
    avg_duration abstime,
    runs integer
);


ALTER TABLE users OWNER TO postgres;

--
-- TOC entry 2036 (class 2604 OID 16402)
-- Name: edges eid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edges ALTER COLUMN eid SET DEFAULT nextval('edges_eid_seq'::regclass);


--
-- TOC entry 2035 (class 2604 OID 16394)
-- Name: nodes nid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY nodes ALTER COLUMN nid SET DEFAULT nextval('nodes_nid_seq'::regclass);


--
-- TOC entry 2038 (class 2604 OID 16428)
-- Name: pois pid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY pois ALTER COLUMN pid SET DEFAULT nextval('pois_pid_seq'::regclass);


--
-- TOC entry 2039 (class 2604 OID 16440)
-- Name: routes rid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY routes ALTER COLUMN rid SET DEFAULT nextval('routes_rid_seq'::regclass);


--
-- TOC entry 2043 (class 2606 OID 16407)
-- Name: edges edges_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edges
    ADD CONSTRAINT edges_pkey PRIMARY KEY (eid);


--
-- TOC entry 2041 (class 2606 OID 16396)
-- Name: nodes nodes_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY nodes
    ADD CONSTRAINT nodes_pkey PRIMARY KEY (nid);


--
-- TOC entry 2045 (class 2606 OID 16433)
-- Name: pois pois_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY pois
    ADD CONSTRAINT pois_pkey PRIMARY KEY (pid);


--
-- TOC entry 2047 (class 2606 OID 16445)
-- Name: routes routes_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY routes
    ADD CONSTRAINT routes_pkey PRIMARY KEY (rid);


--
-- TOC entry 2049 (class 2606 OID 16450)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY users
    ADD CONSTRAINT users_pkey PRIMARY KEY (uid);


--
-- TOC entry 2050 (class 2606 OID 16413)
-- Name: edge_nodes edges_fkey; Type: FK CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edge_nodes
    ADD CONSTRAINT edges_fkey FOREIGN KEY (eid) REFERENCES edges(eid) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- TOC entry 2051 (class 2606 OID 16418)
-- Name: edge_nodes nodes_fkey; Type: FK CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edge_nodes
    ADD CONSTRAINT nodes_fkey FOREIGN KEY (nid) REFERENCES nodes(nid);


-- Completed on 2017-07-12 13:31:10 CEST

--
-- PostgreSQL database dump complete
--

