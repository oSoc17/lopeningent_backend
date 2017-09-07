--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.3
-- Dumped by pg_dump version 9.6.3

-- Started on 2017-07-19 16:24:16 CEST

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 5 (class 2615 OID 16388)
-- Name: lopeningent; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA lopeningent;


ALTER SCHEMA lopeningent OWNER TO postgres;

SET search_path = lopeningent, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 189 (class 1259 OID 16399)
-- Name: edges; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE edges (
    eid integer NOT NULL,
    rating real,
    tags character varying(128)[],
    to_node integer,
    from_node integer,
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
-- TOC entry 2159 (class 0 OID 0)
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
    coord point,
    poi_id integer[]
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
-- TOC entry 2160 (class 0 OID 0)
-- Dependencies: 186
-- Name: nodes_nid_seq; Type: SEQUENCE OWNED BY; Schema: lopeningent; Owner: postgres
--

ALTER SEQUENCE nodes_nid_seq OWNED BY nodes.nid;


--
-- TOC entry 191 (class 1259 OID 16425)
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
-- TOC entry 190 (class 1259 OID 16423)
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
-- TOC entry 2161 (class 0 OID 0)
-- Dependencies: 190
-- Name: pois_pid_seq; Type: SEQUENCE OWNED BY; Schema: lopeningent; Owner: postgres
--

ALTER SEQUENCE pois_pid_seq OWNED BY pois.pid;


--
-- TOC entry 192 (class 1259 OID 16446)
-- Name: users; Type: TABLE; Schema: lopeningent; Owner: postgres
--

CREATE TABLE users (
    uid varchar(256) NOT NULL,
    avg_speed real,
    avg_heartrate integer,
    avg_distance integer,
    tot_distance integer,
    tot_duration integer,
    avg_duration integer,
    runs integer,
    edit_time bigint
);


ALTER TABLE users OWNER TO postgres;

--
-- TOC entry 2025 (class 2604 OID 16402)
-- Name: edges eid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edges ALTER COLUMN eid SET DEFAULT nextval('edges_eid_seq'::regclass);


--
-- TOC entry 2024 (class 2604 OID 16394)
-- Name: nodes nid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY nodes ALTER COLUMN nid SET DEFAULT nextval('nodes_nid_seq'::regclass);


--
-- TOC entry 2027 (class 2604 OID 16428)
-- Name: pois pid; Type: DEFAULT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY pois ALTER COLUMN pid SET DEFAULT nextval('pois_pid_seq'::regclass);


--
-- TOC entry 2031 (class 2606 OID 16407)
-- Name: edges edges_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edges
    ADD CONSTRAINT edges_pkey PRIMARY KEY (eid);


--
-- TOC entry 2029 (class 2606 OID 16396)
-- Name: nodes nodes_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY nodes
    ADD CONSTRAINT nodes_pkey PRIMARY KEY (nid);


--
-- TOC entry 2033 (class 2606 OID 16433)
-- Name: pois pois_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY pois
    ADD CONSTRAINT pois_pkey PRIMARY KEY (pid);


--
-- TOC entry 2035 (class 2606 OID 16450)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY users
    ADD CONSTRAINT users_pkey PRIMARY KEY (uid);


--
-- TOC entry 2036 (class 2606 OID 17072)
-- Name: edges from_fkey; Type: FK CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edges
    ADD CONSTRAINT from_fkey FOREIGN KEY (from_node) REFERENCES nodes(nid);


--
-- TOC entry 2037 (class 2606 OID 17077)
-- Name: edges to_fkey; Type: FK CONSTRAINT; Schema: lopeningent; Owner: postgres
--

ALTER TABLE ONLY edges
    ADD CONSTRAINT to_fkey FOREIGN KEY (to_node) REFERENCES nodes(nid);


-- Completed on 2017-07-19 16:24:16 CEST

--
-- PostgreSQL database dump complete
--

